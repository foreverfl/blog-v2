# =============================================================================
# Compute-EC2 Module - Resource Creation
# =============================================================================
# Currently query-only (data sources only)
# EC2 instance creation will be added here (map-based for_each)
# =============================================================================

# TODO: EC2 Instance Creation (map-based)
# resource "aws_instance" "this" {
#   for_each = var.instances
#
#   ami                         = data.aws_ami.latest.id
#   instance_type               = each.value.instance_type
#   subnet_id                   = each.value.subnet_id
#   vpc_security_group_ids      = each.value.security_group_ids
#   key_name                    = each.value.key_name
#   associate_public_ip_address = each.value.is_public
#   user_data                   = each.value.user_data
#
#   root_block_device {
#     volume_size = each.value.root_volume_size
#     volume_type = "gp3"
#     encrypted   = true
#   }
#
#   tags = merge(
#     {
#       Name = each.key
#     },
#     each.value.tags
#   )
# }

# Usage example (live/prod/aws/terraform.tfvars):
# instances = {
#   "public-web" = {
#     instance_type      = "t3.micro"
#     subnet_id          = "subnet-xxx"  # public subnet
#     is_public          = true
#     security_group_ids = ["sg-xxx"]
#     key_name           = "my-key"
#     root_volume_size   = 20
#     tags = {
#       Role = "web"
#     }
#   }
#   "private-app" = {
#     instance_type      = "t3.small"
#     subnet_id          = "subnet-yyy"  # private subnet
#     is_public          = false
#     security_group_ids = ["sg-yyy"]
#     key_name           = "my-key"
#     root_volume_size   = 30
#     tags = {
#       Role = "application"
#     }
#   }
# }