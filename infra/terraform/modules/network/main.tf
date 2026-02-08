# =============================================================================
# Network Module - Resource Creation
# =============================================================================
# Currently query-only (data sources only)
# NAT Gateway toggle and other resources will be added here
# =============================================================================

# TODO: NAT Gateway Toggle
# resource "aws_eip" "nat" {
#   count  = var.enable_nat ? 1 : 0
#   domain = "vpc"
#   tags = {
#     Name = "nat-eip"
#   }
# }
#
# resource "aws_nat_gateway" "main" {
#   count         = var.enable_nat ? 1 : 0
#   allocation_id = aws_eip.nat[0].id
#   subnet_id     = var.nat_subnet_id
#   tags = {
#     Name = "main-nat-gateway"
#   }
# }