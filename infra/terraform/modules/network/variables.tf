# =============================================================================
# Network Module Variables
# =============================================================================
# VPC, Subnets, Internet Gateway, NAT Gateway, Route Tables
# =============================================================================

variable "vpc_id" {
  description = "VPC ID to query (optional - uses first VPC if not specified)"
  type        = string
  default     = ""
}

# =============================================================================
# NAT Gateway Toggle (for resource creation)
# =============================================================================

variable "enable_nat" {
  description = "Enable NAT Gateway for private subnets"
  type        = bool
  default     = false
}

variable "nat_subnet_id" {
  description = "Public subnet ID where NAT Gateway will be created"
  type        = string
  default     = ""
}