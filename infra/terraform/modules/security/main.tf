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

# =============================================================================
# Database Ingress Rules (add to existing SG)
# =============================================================================

locals {
  db_instance_name  = "parapara-postgres"
  app_instance_name = "parapara-server"
  db_postgres_port  = 5431
  db_redis_port     = 6379
}

data "aws_instance" "db" {
  filter {
    name   = "tag:Name"
    values = [local.db_instance_name]
  }

  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }
}

data "aws_instance" "app" {
  filter {
    name   = "tag:Name"
    values = [local.app_instance_name]
  }

  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }
}

resource "aws_vpc_security_group_ingress_rule" "db_postgres" {
  security_group_id = tolist(data.aws_instance.db.vpc_security_group_ids)[0]
  from_port         = local.db_postgres_port
  to_port           = local.db_postgres_port
  ip_protocol       = "tcp"
  cidr_ipv4         = "${data.aws_instance.app.private_ip}/32"
  description       = "PostgreSQL from ${local.app_instance_name}"
}

resource "aws_vpc_security_group_ingress_rule" "db_redis" {
  security_group_id = tolist(data.aws_instance.db.vpc_security_group_ids)[0]
  from_port         = local.db_redis_port
  to_port           = local.db_redis_port
  ip_protocol       = "tcp"
  cidr_ipv4         = "${data.aws_instance.app.private_ip}/32"
  description       = "Redis from ${local.app_instance_name}"
}