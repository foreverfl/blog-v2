# =============================================================================
# Security Module - Resource Creation
# =============================================================================
# Currently query-only (data sources only)
# Security Groups, IAM resources will be added here
# =============================================================================

# TODO: Default Security Groups
# resource "aws_security_group" "web" {
#   count       = var.create_default_sgs ? 1 : 0
#   name        = "web-sg"
#   description = "Security group for web servers"
#   vpc_id      = var.vpc_id
#
#   ingress {
#     from_port   = 80
#     to_port     = 80
#     protocol    = "tcp"
#     cidr_blocks = var.allowed_web_cidrs
#   }
#
#   ingress {
#     from_port   = 443
#     to_port     = 443
#     protocol    = "tcp"
#     cidr_blocks = var.allowed_web_cidrs
#   }
#
#   egress {
#     from_port   = 0
#     to_port     = 0
#     protocol    = "-1"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
#
#   tags = {
#     Name = "web-sg"
#   }
# }

# TODO: SSH Security Group
# resource "aws_security_group" "ssh" {
#   count       = var.create_default_sgs && length(var.allowed_ssh_cidrs) > 0 ? 1 : 0
#   name        = "ssh-sg"
#   description = "Security group for SSH access"
#   vpc_id      = var.vpc_id
#
#   ingress {
#     from_port   = 22
#     to_port     = 22
#     protocol    = "tcp"
#     cidr_blocks = var.allowed_ssh_cidrs
#   }
#
#   egress {
#     from_port   = 0
#     to_port     = 0
#     protocol    = "-1"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
#
#   tags = {
#     Name = "ssh-sg"
#   }
# }