# =============================================================================
# Prod AWS Infrastructure
# =============================================================================

# =============================================================================
# 1. Network Module
# =============================================================================

module "network" {
  source = "../../../modules/network"

  vpc_id = var.vpc_id
}

# =============================================================================
# 2. Security Module (비활성)
# =============================================================================
# module "security" {
#   source = "../../../modules/security"
#
#   vpc_id            = module.network.vpcs[var.vpc_id].name  # TODO: 구조 변경 후 수정
#   allowed_ssh_cidrs = var.allowed_ssh_cidrs
# }

# =============================================================================
# 3. Compute-EC2 Module (비활성)
# =============================================================================
# module "compute" {
#   source = "../../../modules/compute-ec2"
#
#   vpc_id          = ...
#   include_stopped = var.include_stopped_instances
# }

# =============================================================================
# 4. Observability Module (비활성)
# =============================================================================
# module "observability" {
#   source = "../../../modules/observability"
#
#   log_group_prefix = var.log_group_prefix
#   ec2_instance_ids = module.compute.instances_summary.instance_ids
# }

# =============================================================================
# Outputs - Network
# =============================================================================

output "network_vpcs" {
  description = "VPC details"
  value       = module.network.vpcs
}

output "network_subnet_details" {
  description = "Subnet details"
  value       = module.network.subnet_details
}

output "network_route_tables" {
  description = "Route table details"
  value       = module.network.route_table_details
}

output "network_availability_zones" {
  description = "Available AZs"
  value       = module.network.availability_zones
}

output "network_summary" {
  description = "Network summary"
  value       = module.network.summary
}