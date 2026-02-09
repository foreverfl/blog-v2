# =============================================================================
# VPCs
# =============================================================================

output "vpcs" {
  description = "All queried VPCs with their network details"
  value = {
    for vpc_id in local.target_vpc_ids : vpc_id => {
      name             = lookup(data.aws_vpc.details[vpc_id].tags, "Name", "unnamed")
      cidr_block       = data.aws_vpc.details[vpc_id].cidr_block
      dns_hostnames    = data.aws_vpc.details[vpc_id].enable_dns_hostnames
      dns_support      = data.aws_vpc.details[vpc_id].enable_dns_support
      main_route_table = data.aws_vpc.details[vpc_id].main_route_table_id

      subnets = {
        total   = length(data.aws_subnets.by_vpc[vpc_id].ids)
        public  = length(data.aws_subnets.public_by_vpc[vpc_id].ids)
        private = length(data.aws_subnets.private_by_vpc[vpc_id].ids)
      }

      nat_enabled     = length(data.aws_nat_gateways.by_vpc[vpc_id].ids) > 0
      nat_gateway_ids = data.aws_nat_gateways.by_vpc[vpc_id].ids
      route_tables    = length(data.aws_route_tables.by_vpc[vpc_id].ids)
    }
  }
}

# =============================================================================
# Subnet Details
# =============================================================================

output "subnet_details" {
  description = "Detailed subnet information with associated route table"
  value = {
    for id, subnet in data.aws_subnet.details : id => {
      vpc_id            = subnet.vpc_id
      availability_zone = subnet.availability_zone
      cidr_block        = subnet.cidr_block
      available_ips     = subnet.available_ip_address_count
      is_public         = subnet.map_public_ip_on_launch
      route_table_id    = data.aws_route_table.by_subnet[id].route_table_id
      tags              = subnet.tags
    }
  }
}

# =============================================================================
# Route Table Details
# =============================================================================

output "route_table_details" {
  description = "Detailed route table information with associated subnets"
  value = {
    for id, rt in data.aws_route_table.details : id => {
      vpc_id = rt.vpc_id
      routes = rt.routes
      tags   = rt.tags
      associated_subnets = [
        for subnet_id in local.all_subnet_ids :
        subnet_id if data.aws_route_table.by_subnet[subnet_id].route_table_id == id
      ]
    }
  }
}

# =============================================================================
# NAT Gateway
# =============================================================================

output "nat_gateway" {
  description = "NAT Gateway details (null when disabled)"
  value = local.nat_create ? {
    enabled         = true
    nat_gateway_id  = aws_nat_gateway.main[0].id
    eip_public_ip   = aws_eip.nat[0].public_ip
    eip_id          = aws_eip.nat[0].id
    subnet_id       = var.nat_subnet_id
    route_table_ids = toset(var.nat_private_route_table_ids)
  } : {
    enabled         = false
    nat_gateway_id  = null
    eip_public_ip   = null
    eip_id          = null
    subnet_id       = null
    route_table_ids = toset([])
  }
}

# =============================================================================
# Availability Zones
# =============================================================================

output "availability_zones" {
  description = "Available AZs in the region"
  value       = data.aws_availability_zones.available.names
}

# =============================================================================
# Summary
# =============================================================================

output "summary" {
  description = "Network infrastructure summary"
  value = {
    total_vpcs    = length(local.target_vpc_ids)
    total_subnets = length(local.all_subnet_ids)
    vpc_names = {
      for vpc_id in local.target_vpc_ids :
      vpc_id => lookup(data.aws_vpc.details[vpc_id].tags, "Name", "unnamed")
    }
  }
}