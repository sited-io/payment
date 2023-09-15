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
            upstreams {
              destination_name = "cockroach-sql"
              local_bind_port  = 5432
            }
            upstreams {
              destination_name = "commerce-api"
              local_bind_port  = 10000
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
{{ with nomadVar "nomad/jobs/payment" }}
RUST_LOG='{{ .LOG_LEVEL }}'
{{ end }}

HOST='0.0.0.0:{{ env "NOMAD_PORT_grpc" }}'

DB_HOST='{{ env "NOMAD_UPSTREAM_IP_cockroach-sql" }}'
DB_PORT='{{ env "NOMAD_UPSTREAM_PORT_cockroach-sql" }}'
DB_DBNAME='payment'
DB_USER='payment_user'
{{ with secret "database/static-creds/payment_user" }}
DB_PASSWORD='{{ .Data.password }}'
{{ end }}

{{ with nomadVar "nomad/jobs/" }}
JWKS_HOST='{{ .JWKS_HOST }}'
{{ end }}
JWKS_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_zitadel" }}/oauth/v2/keys'

COMMERCE_SERVICE_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_commerce-api" }}'

{{ with secret "kv2/data/services/payment" }}
STRIPE_SECRET_KEY='{{ .Data.data.STRIPE_SECRET_KEY }}'
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
