# =============================================================================
# Network Module Variables
# =============================================================================

variable "vpc_id" {
  description = "Specific VPC ID to query. Empty = query all VPCs."
  type        = string
  default     = ""
}