# =============================================================================
# Cloudflare DNS Module Variables
# =============================================================================
# DNS records, Zone settings query and management
# =============================================================================

variable "zone_name" {
  description = "Cloudflare zone (domain) name to manage"
  type        = string
}

# =============================================================================
# DNS Record Filters (for query)
# =============================================================================

variable "record_type_filter" {
  description = "Filter DNS records by type (A, AAAA, CNAME, TXT, MX, etc.)"
  type        = list(string)
  default     = []  # Empty = all types
}

# =============================================================================
# Resource Creation Variables (for future use)
# =============================================================================

variable "dns_records" {
  description = "Map of DNS records to create"
  type = map(object({
    type    = string
    name    = string
    content = string
    ttl     = optional(number, 1)  # 1 = Auto
    proxied = optional(bool, false)
  }))
  default = {}
}

variable "enable_proxy_by_default" {
  description = "Enable Cloudflare proxy (orange cloud) by default"
  type        = bool
  default     = true
}