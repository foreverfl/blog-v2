terraform {
  required_version = ">= 1.0.0"

  cloud {
    organization = "mogumogu"

    workspaces {
      name = "blog-v2-cloudflare"
    }
  }

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}