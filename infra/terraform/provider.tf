terraform {
  required_version = ">= 1.0.0"

  cloud {
    organization = "mogumogu"

    workspaces {
      name = "blog-v2"
    }
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region

  default_tags {
    tags = {
      ManagedBy = "Terraform"
      Project   = "blog-v2"
    }
  }
}
