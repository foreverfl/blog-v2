# =============================================================================
# VPC
# =============================================================================

output "vpc_id" {
  description = "Selected VPC ID"
  value       = local.has_vpc ? local.target_vpc_id : null
}

output "vpc" {
  description = "VPC information"
  value = length(data.aws_vpc.selected) > 0 ? {
    id                   = data.aws_vpc.selected[0].id
    cidr_block           = data.aws_vpc.selected[0].cidr_block
    enable_dns_hostnames = data.aws_vpc.selected[0].enable_dns_hostnames
    enable_dns_support   = data.aws_vpc.selected[0].enable_dns_support
    main_route_table_id  = data.aws_vpc.selected[0].main_route_table_id
    tags                 = data.aws_vpc.selected[0].tags
  } : null
}

# =============================================================================
# Availability Zones
# =============================================================================

output "availability_zones" {
  description = "Available AZs in the region"
  value       = data.aws_availability_zones.available.names
}

# =============================================================================
# Subnets
# =============================================================================

output "public_subnet_ids" {
  description = "List of public subnet IDs"
  value       = length(data.aws_subnets.public) > 0 ? data.aws_subnets.public[0].ids : []
}

output "private_subnet_ids" {
  description = "List of private subnet IDs"
  value       = length(data.aws_subnets.private) > 0 ? data.aws_subnets.private[0].ids : []
}

output "subnets_summary" {
  description = "Subnets summary"
  value = {
    total_count   = length(data.aws_subnets.all) > 0 ? length(data.aws_subnets.all[0].ids) : 0
    public_count  = length(data.aws_subnets.public) > 0 ? length(data.aws_subnets.public[0].ids) : 0
    private_count = length(data.aws_subnets.private) > 0 ? length(data.aws_subnets.private[0].ids) : 0
    public_ids    = length(data.aws_subnets.public) > 0 ? data.aws_subnets.public[0].ids : []
    private_ids   = length(data.aws_subnets.private) > 0 ? data.aws_subnets.private[0].ids : []
  }
}

output "subnet_details" {
  description = "Detailed subnet information"
  value = {
    for id, subnet in data.aws_subnet.details : id => {
      id                      = subnet.id
      availability_zone       = subnet.availability_zone
      cidr_block              = subnet.cidr_block
      available_ip_count      = subnet.available_ip_address_count
      map_public_ip_on_launch = subnet.map_public_ip_on_launch
      tags                    = subnet.tags
    }
  }
}

# =============================================================================
# Gateways
# =============================================================================

output "internet_gateway" {
  description = "Internet Gateway information"
  value = length(data.aws_internet_gateway.main) > 0 ? {
    id     = data.aws_internet_gateway.main[0].id
    vpc_id = data.aws_internet_gateway.main[0].attachments[0].vpc_id
    state  = data.aws_internet_gateway.main[0].attachments[0].state
    tags   = data.aws_internet_gateway.main[0].tags
  } : null
}

output "nat_gateways" {
  description = "NAT Gateways"
  value = {
    count = length(data.aws_nat_gateways.all) > 0 ? length(data.aws_nat_gateways.all[0].ids) : 0
    ids   = length(data.aws_nat_gateways.all) > 0 ? data.aws_nat_gateways.all[0].ids : []
  }
}

output "nat_enabled" {
  description = "Whether NAT Gateway is currently active"
  value       = length(data.aws_nat_gateways.all) > 0 ? length(data.aws_nat_gateways.all[0].ids) > 0 : false
}

# =============================================================================
# Route Tables
# =============================================================================

output "route_tables" {
  description = "Route Tables summary"
  value = {
    count = length(data.aws_route_tables.all) > 0 ? length(data.aws_route_tables.all[0].ids) : 0
    ids   = length(data.aws_route_tables.all) > 0 ? data.aws_route_tables.all[0].ids : []
  }
}

output "route_table_details" {
  description = "Detailed route table information"
  value = {
    for id, rt in data.aws_route_table.details : id => {
      id        = rt.route_table_id
      vpc_id    = rt.vpc_id
      routes    = rt.routes
      tags      = rt.tags
    }
  }
}