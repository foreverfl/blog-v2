# =============================================================================
# Network Module Variables
# =============================================================================

variable "vpc_id" {
  description = "Specific VPC ID to query. Empty = query all VPCs."
  type        = string
  default     = ""
}

variable "nat_enabled" {
  description = "Create a NAT Gateway for private subnet internet access. Requires vpc_id to be set."
  type        = bool
  default     = false
}

variable "nat_subnet_id" {
  description = "Subnet ID to place the NAT Gateway in (must have IGW route). Required when nat_enabled = true."
  type        = string
  default     = ""
}

variable "nat_private_route_table_ids" {
  description = "Route table IDs of private subnets to add NAT route to. Required when nat_enabled = true."
  type        = list(string)
  default     = []
}