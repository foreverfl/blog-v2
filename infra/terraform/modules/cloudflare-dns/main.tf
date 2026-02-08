# =============================================================================
# Cloudflare DNS Module - Resource Creation
# =============================================================================
# Currently query-only (data sources only)
# DNS record management will be added here
# =============================================================================

# TODO: DNS Record Creation (map-based)
# resource "cloudflare_dns_record" "this" {
#   for_each = var.dns_records
#
#   zone_id = data.cloudflare_zone.this.id
#   type    = each.value.type
#   name    = each.value.name
#   content = each.value.content
#   ttl     = each.value.ttl
#   proxied = each.value.proxied
# }

# Usage example (live/prod/cloudflare/terraform.tfvars):
# dns_records = {
#   "blog-root" = {
#     type    = "A"
#     name    = "@"
#     content = "1.2.3.4"  # Your server IP
#     proxied = true
#   }
#   "blog-www" = {
#     type    = "CNAME"
#     name    = "www"
#     content = "example.com"
#     proxied = true
#   }
#   "api" = {
#     type    = "A"
#     name    = "api"
#     content = "1.2.3.4"
#     proxied = true
#   }
# }