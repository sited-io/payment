job "payment" {
  datacenters = ["dc1"]
  type        = "service"

  group "payment-api" {
    count = 2

    network {
      mode = "bridge"

      port "grpc" {}
    }

    service {
      name = "payment-api"
      port = "grpc"

      connect {
        sidecar_service {
          proxy {
            upstreams {
              destination_name = "zitadel"
              local_bind_port  = 8080
            }
          }
        }
      }

      check {
        type     = "grpc"
        interval = "20s"
        timeout  = "2s"
      }
    }

    task "payment-api" {
      driver = "docker"

      vault {
        policies = ["service-payment"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
HOST='0.0.0.0:{{ env "NOMAD_PORT_grpc" }}'

{{ with nomadVar "nomad/jobs/" }}
JWKS_HOST='{{ .JWKS_HOST }}'
{{ end }}
JWKS_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_zitadel" }}/oauth/v2/keys'

{{ with nomadVar "nomad/jobs/payment" }}
RUST_LOG='{{ .LOG_LEVEL }}'
{{ end }}

ZITADEL_BASE_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_zitadel" }}'
ZITADEL_CLIENT_ID='payment_service'
{{ with secret "kv2/data/services/payment" }}
ZITADEL_CLIENT_SECRET='{{ .Data.data.ZITADEL_CLIENT_SECRET }}'
{{ end }}
EOF
      }

      config {
        image      = "__IMAGE__"
        force_pull = true
      }
    }
  }
}
