# =============================================================================
# VPC
# =============================================================================

data "aws_vpcs" "all" {}

locals {
  target_vpc_id = var.vpc_id != "" ? var.vpc_id : (
    length(data.aws_vpcs.all.ids) > 0 ? data.aws_vpcs.all.ids[0] : ""
  )
  has_vpc = local.target_vpc_id != ""
}

data "aws_vpc" "selected" {
  count = local.has_vpc ? 1 : 0
  id    = local.target_vpc_id
}

# =============================================================================
# Availability Zones
# =============================================================================

data "aws_availability_zones" "available" {
  state = "available"
}

# =============================================================================
# Subnets
# =============================================================================

data "aws_subnets" "all" {
  count = local.has_vpc ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [local.target_vpc_id]
  }
}

data "aws_subnets" "public" {
  count = local.has_vpc ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [local.target_vpc_id]
  }

  filter {
    name   = "map-public-ip-on-launch"
    values = ["true"]
  }
}

data "aws_subnets" "private" {
  count = local.has_vpc ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [local.target_vpc_id]
  }

  filter {
    name   = "map-public-ip-on-launch"
    values = ["false"]
  }
}

data "aws_subnet" "details" {
  for_each = local.has_vpc ? toset(data.aws_subnets.all[0].ids) : toset([])
  id       = each.value
}

# =============================================================================
# Internet Gateway
# =============================================================================

data "aws_internet_gateway" "main" {
  count = local.has_vpc ? 1 : 0

  filter {
    name   = "attachment.vpc-id"
    values = [local.target_vpc_id]
  }
}

# =============================================================================
# NAT Gateways
# =============================================================================

data "aws_nat_gateways" "all" {
  count = local.has_vpc ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [local.target_vpc_id]
  }

  filter {
    name   = "state"
    values = ["available", "pending"]
  }
}

# =============================================================================
# Route Tables
# =============================================================================

data "aws_route_tables" "all" {
  count = local.has_vpc ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [local.target_vpc_id]
  }
}

data "aws_route_table" "details" {
  for_each = local.has_vpc ? toset(data.aws_route_tables.all[0].ids) : toset([])
  route_table_id = each.value
}