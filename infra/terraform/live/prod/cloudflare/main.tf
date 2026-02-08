# =============================================================================
# Prod Cloudflare Infrastructure
# =============================================================================
# DNS and zone settings management
# =============================================================================

module "cloudflare_dns" {
  source = "../../../modules/cloudflare-dns"

  zone_name = var.zone_name
}

# =============================================================================
# Outputs
# =============================================================================

output "zone_info" {
  description = "Cloudflare zone information"
  value       = module.cloudflare_dns.zone_info
}

output "zone_id" {
  description = "Cloudflare zone ID (use this for DNS record management)"
  value       = module.cloudflare_dns.zone_id
}

output "cloudflare_overview" {
  description = "Cloudflare configuration overview"
  value       = module.cloudflare_dns.cloudflare_overview
}