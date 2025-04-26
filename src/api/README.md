# Keep-Your-Contacts API Documentation

## Overview

Keep-Your-Contacts is a RESTful API for managing contact information in a professional context. It enables users to store, retrieve, and manage contact details including personal information, professional background, and location data.

**Current Version**: v1.0.0

## Authentication and Authorization

Authentication is handled via JWT (JSON Web Tokens) with secure httpOnly cookies.

### Registration

```bash
curl -X POST http://api.keepyourcontacts.com/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "john_doe",
    "email": "john@example.com",
    "password": "secure_password123"
  }'
```

### Login

```bash
curl -X POST http://api.keepyourcontacts.com/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "name": "john_doe",
    "password": "secure_password123"
  }'
```

## Base URL and Endpoint Structure

Base URL (this is only for the examples listed here): `http://api.keepyourcontacts.com`

### Core Endpoints

- `/auth/*` - Authentication operations
- `/persons` - Contact management
- `/known-from-sources` - Known from source management

## Request and Response Format

All requests and responses use JSON format.

### Persons Endpoints

#### Get Persons List

```bash
curl -X GET "http://api.keepyourcontacts.com/persons?page=0&per_page=10&detailed=true" \
  -H "Cookie: token=<your-jwt-token>"
```

Response:

```json
{
  "persons": [
    {
      "id": 1,
      "first_name": "John",
      "last_name": "Doe",
      "job_title": "Software Engineer",
      "company": "Tech Corp",
      "linkedin": "john-doe",
      "notes": "Met at conference",
      "record": {
        "name": "San Francisco",
        "country": "United States"
      }
    }
  ]
}
```

#### Create Person

```bash
curl -X POST http://api.keepyourcontacts.com/persons \
  -H "Content-Type: application/json" \
  -H "Cookie: token=<your-jwt-token>" \
  -d '{
    "name": "John Doe",
    "known_from_source_id": 1,
    "job_title": "Software Engineer",
    "company": "Tech Corp"
  }'
```

## HTTP Methods and Status Codes

### Supported Methods

- `GET` - Retrieve resources
- `POST` - Create resources
- `PUT` - Update resources
- `DELETE` - Remove resources

### Common Status Codes

- 200 OK - Success
- 201 Created - Resource created successfully
- 400 Bad Request - Invalid parameters
- 401 Unauthorized - Missing/invalid authentication
- 403 Forbidden - Insufficient permissions
- 404 Not Found - Resource not found
- 500 Internal Server Error - Server error

## Detailed Endpoint Documentation

### Authentication

#### POST /auth/register

Creates new user account.

Required fields:

- name: string
- email: string
- password: string

#### POST /auth/login

Authenticates user and returns JWT token.

Required fields:

- name: string
- password: string

#### GET /auth/me

Returns authenticated user profile.

### Persons

#### GET /persons

Lists contacts with pagination and filtering options.

Query Parameters:

- `page` (required, integer): Zero-based page number
- `per_page` (required, integer): Number of items per page
- `detailed` (optional, boolean): When true, returns additional fields:
  - job_title
  - company
  - linkedin
  - notes
  - created_at
- `global_search` (optional, string): Searches across all text fields
- `known_from_search` (optional, integer): Filters contacts by their source ID

Example request with all parameters:

```bash
curl -X GET "http://api.keepyourcontacts.com/persons\
?page=0\
&per_page=10\
&detailed=true\
&global_search=engineer\
&known_from_search=1" \
-H "Cookie: token=<your-jwt-token>"
```

Simple response (detailed=false):

```json
{
  "persons": [
    {
      "id": 1,
      "first_name": "John",
      "last_name": "Doe",
      "record": {
        "name": "San Francisco",
        "country": "United States"
      }
    }
  ]
}
```

Detailed response (detailed=true):

```json
{
  "persons": [
    {
      "id": 1,
      "first_name": "John",
      "last_name": "Doe",
      "job_title": "Software Engineer",
      "company": "Tech Corp",
      "linkedin": "john-doe",
      "notes": "Met at conference",
      "created_at": "2024-01-20T15:00:00Z",
      "record": {
        "name": "San Francisco",
        "country": "United States"
      }
    }
  ]
}
```

### Known From Sources

#### GET /known-from-sources

Lists all contact sources.

#### POST /known-from-sources

Creates new contact source.

Required fields:

- source_name: string
- description: string

## Error Handling

Errors are returned as StatusCodes which are not OK. A detailed description is returned in the response body in text format.

## Changelog

### v1.0.0 (2024)

- Initial API release
- JWT authentication
- Basic CRUD operations for contacts
- Contact source management
- Location-based features
- Search through contact fields

## Missing features

- Post and delete Known-From-Sources
- Scraping LinkedIn for profile pictures?
  The database structure will change.

## Glossary

- JWT: JSON Web Token used for authentication
- Contact Source: Origin or context where you met a contact
- Coordinate: Geographic location (latitude/longitude)
