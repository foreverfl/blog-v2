# =============================================================================
# Cloudflare Provider
# =============================================================================
# Authentication via environment variable: CLOUDFLARE_API_TOKEN
# Set in Terraform Cloud workspace variables (sensitive)
# =============================================================================

provider "cloudflare" {
  # API token is read from CLOUDFLARE_API_TOKEN environment variable
}