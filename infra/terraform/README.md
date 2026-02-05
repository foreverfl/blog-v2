# Terraform AWS Infrastructure Query

Query your AWS infrastructure information using Terraform data sources with HCP Terraform Cloud backend.

## Prerequisites

- Terraform >= 1.0.0
- HCP Terraform Cloud account (https://app.terraform.io)
- AWS IAM credentials with read permissions

## Setup

### 1. Terraform Cloud Login

```bash
terraform login
```

A browser will open. Generate a token and paste it into the terminal.

### 2. Terraform Cloud Workspace Configuration

At [app.terraform.io](https://app.terraform.io):

1. Create or select an Organization (current: `mogumogu`)
2. Create Workspace: `blog-v2`
3. In the **Variables** tab, configure AWS credentials:

| Variable | Category | Sensitive |
|----------|----------|-----------|
| `AWS_ACCESS_KEY_ID` | **Environment variable** | No |
| `AWS_SECRET_ACCESS_KEY` | **Environment variable** | Yes |
| `AWS_REGION` | **Environment variable** | No |

> **Important**: Category must be set to **Environment variable**, NOT Terraform variable!

### 3. Initialize & Apply

```bash
cd infra/terraform

# Initialize (connect to Terraform Cloud)
terraform init

# Preview
terraform plan

# Apply
terraform apply
```

## Available Outputs

| Output | Description |
|--------|-------------|
| `account_info` | AWS account ID, ARN, region |
| `availability_zones` | Available AZs in the region |
| `vpc_info` | VPC details (CIDR, DNS settings, etc.) |
| `subnets_summary` | Public/Private subnet counts and IDs |
| `subnet_details` | Detailed info for each subnet |
| `internet_gateway` | IGW information |
| `nat_gateways` | NAT Gateway IDs |
| `route_tables` | Route table IDs |
| `ec2_instances_summary` | Total instance count and IDs |
| `ec2_instance_details` | Full details for each EC2 instance |
| `security_groups_summary` | Security group count and IDs |
| `security_group_details` | Security group basic info |
| `ebs_volumes_summary` | EBS volume count and IDs |
| `ebs_volume_details` | Volume size, type, encryption |
| `elastic_ips` | Elastic IP addresses |

## Usage

```bash
# View specific output
terraform output ec2_instance_details
terraform output vpc_info

# JSON format (useful for scripting)
terraform output -json ec2_instance_details

# View in Terraform Cloud UI
# https://app.terraform.io/app/mogumogu/blog-v2
```

## Custom Region

Add `aws_region` variable in Terraform Cloud Variables:

| Variable | Category | Value |
|----------|----------|-------|
| `aws_region` | Terraform variable | `ap-northeast-1` |

## Architecture

```
Local Machine          HCP Terraform Cloud
┌─────────────┐       ┌──────────────────────┐
│ terraform   │──────▶│ Workspace: blog-v2   │
│ plan/apply  │       │                      │
└─────────────┘       │ ┌──────────────────┐ │
                      │ │ AWS Credentials  │ │
                      │ │ (Env Variables)  │ │
                      │ └──────────────────┘ │
                      │          │           │
                      │          ▼           │
                      │ ┌──────────────────┐ │
                      │ │ Execute Plan/    │ │
                      │ │ Apply on Cloud   │ │
                      │ └──────────────────┘ │
                      └──────────────────────┘
```

## Notes

- This configuration only uses data sources, so no AWS resources are created or modified
- State is stored in Terraform Cloud
- Plan/Apply runs in the Terraform Cloud environment