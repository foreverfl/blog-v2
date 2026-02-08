# =============================================================================
# Zone Information
# =============================================================================

output "zone_id" {
  description = "Cloudflare Zone ID"
  value       = data.cloudflare_zone.this.id
}

output "zone_info" {
  description = "Zone information"
  value = {
    id           = data.cloudflare_zone.this.id
    name         = data.cloudflare_zone.this.name
    account_id   = data.cloudflare_zone.this.account_id
    name_servers = data.cloudflare_zone.this.name_servers
    status       = data.cloudflare_zone.this.status
  }
}

# =============================================================================
# Cloudflare Overview
# =============================================================================

output "cloudflare_overview" {
  description = "Cloudflare zone overview"
  value = {
    zone_name    = data.cloudflare_zone.this.name
    zone_id      = data.cloudflare_zone.this.id
    zone_status  = data.cloudflare_zone.this.status
    name_servers = data.cloudflare_zone.this.name_servers
    account_id   = data.cloudflare_zone.this.account_id
  }
}

# =============================================================================
# Note: To manage DNS records, use cloudflare_record resource
# =============================================================================
# Example:
#   resource "cloudflare_record" "www" {
#     zone_id = module.cloudflare_dns.zone_id
#     name    = "www"
#     content = "192.168.1.1"
#     type    = "A"
#     proxied = true
#   }
# =============================================================================