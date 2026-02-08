# =============================================================================
# Security Groups Summary
# =============================================================================

output "security_groups_summary" {
  description = "Security Groups summary"
  value = {
    count = length(data.aws_security_groups.all.ids)
    ids   = data.aws_security_groups.all.ids
  }
}

output "security_group_details" {
  description = "Detailed Security Group information"
  value = {
    for id, sg in data.aws_security_group.details : id => {
      id          = sg.id
      name        = sg.name
      description = sg.description
      vpc_id      = sg.vpc_id
      tags        = sg.tags
      ingress = [
        for rule_id, rule in data.aws_vpc_security_group_rule.details : {
          rule_id                   = rule.security_group_rule_id
          from_port                 = rule.from_port
          to_port                   = rule.to_port
          protocol                  = rule.ip_protocol
          cidr_ipv4                 = rule.cidr_ipv4
          cidr_ipv6                 = rule.cidr_ipv6
          referenced_security_group = rule.referenced_security_group_id
          description               = rule.description
        } if rule.security_group_id == id && !rule.is_egress
      ]
      egress = [
        for rule_id, rule in data.aws_vpc_security_group_rule.details : {
          rule_id                   = rule.security_group_rule_id
          from_port                 = rule.from_port
          to_port                   = rule.to_port
          protocol                  = rule.ip_protocol
          cidr_ipv4                 = rule.cidr_ipv4
          cidr_ipv6                 = rule.cidr_ipv6
          referenced_security_group = rule.referenced_security_group_id
          description               = rule.description
        } if rule.security_group_id == id && rule.is_egress
      ]
    }
  }
}

# =============================================================================
# Primary Security Group IDs (if found)
# =============================================================================

output "web_security_group_ids" {
  description = "Web/HTTP related security group IDs"
  value       = data.aws_security_groups.web.ids
}

output "ssh_security_group_ids" {
  description = "SSH/Bastion related security group IDs"
  value       = data.aws_security_groups.ssh.ids
}

output "database_security_group_ids" {
  description = "Database related security group IDs"
  value       = data.aws_security_groups.database.ids
}

# =============================================================================
# IAM Information
# =============================================================================

output "ec2_iam_roles" {
  description = "IAM roles available for EC2"
  value = {
    names = data.aws_iam_roles.ec2_roles.names
    arns  = data.aws_iam_roles.ec2_roles.arns
  }
}

output "ssm_policy_arn" {
  description = "SSM Managed Instance Core policy ARN"
  value       = data.aws_iam_policy.ssm_managed_instance.arn
}

# =============================================================================
# Security Summary
# =============================================================================

output "security_overview" {
  description = "Overall security configuration overview"
  value = {
    total_security_groups = length(data.aws_security_groups.all.ids)
    web_sgs_found         = length(data.aws_security_groups.web.ids)
    ssh_sgs_found         = length(data.aws_security_groups.ssh.ids)
    database_sgs_found    = length(data.aws_security_groups.database.ids)
    ec2_roles_found       = length(data.aws_iam_roles.ec2_roles.names)
  }
}