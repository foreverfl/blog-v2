# =============================================================================
# Network Module - Resource Creation
# =============================================================================

locals {
  nat_create = var.nat_enabled && var.nat_subnet_id != ""
}

# =============================================================================
# NAT Gateway (toggle via nat_enabled)
# =============================================================================

resource "aws_eip" "nat" {
  count  = local.nat_create ? 1 : 0
  domain = "vpc"

  tags = {
    Name = "blog-v2-nat-eip"
  }
}

resource "aws_nat_gateway" "main" {
  count         = local.nat_create ? 1 : 0
  allocation_id = aws_eip.nat[0].id
  subnet_id     = var.nat_subnet_id

  tags = {
    Name = "blog-v2-nat-gateway"
  }
}

# Route 0.0.0.0/0 → NAT Gateway for each private subnet route table
resource "aws_route" "private_nat" {
  for_each = local.nat_create ? toset(var.nat_private_route_table_ids) : toset([])

  route_table_id         = each.value
  destination_cidr_block = "0.0.0.0/0"
  nat_gateway_id         = aws_nat_gateway.main[0].id
}