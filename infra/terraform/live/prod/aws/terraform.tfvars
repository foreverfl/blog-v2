# =============================================================================
# Prod AWS Infrastructure Configuration
# =============================================================================
# Modify this file to change infrastructure settings!
# =============================================================================

# -----------------------------------------------------------------------------
# Provider
# -----------------------------------------------------------------------------
aws_region = "ap-northeast-1"  # Tokyo

# -----------------------------------------------------------------------------
# Network
# -----------------------------------------------------------------------------
# vpc_id = ""  # Leave empty to auto-select first VPC

# NAT Gateway Toggle (for cost saving)
# true  = NAT on (private subnet -> internet enabled)
# false = NAT off (cost saving, private subnet isolated)
enable_nat = false

# -----------------------------------------------------------------------------
# Security
# -----------------------------------------------------------------------------
# SSH access allowed IPs (recommend IP restriction for security)
# allowed_ssh_cidrs = ["your.ip.address/32"]
allowed_ssh_cidrs = []

# -----------------------------------------------------------------------------
# Compute
# -----------------------------------------------------------------------------
# Include stopped instances in query results
include_stopped_instances = true

# -----------------------------------------------------------------------------
# Observability
# -----------------------------------------------------------------------------
# CloudWatch log group filter (empty = all)
log_group_prefix = ""