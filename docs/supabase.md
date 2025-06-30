
# Supabase Integration Guide

WasmRunner uses Supabase for user authentication, app store functionality, and backend services.

## Overview

The integration provides:
- **User Authentication**: Email/password and OAuth login
- **App Store Database**: Apps, versions, reviews, and analytics
- **File Storage**: WASM binaries and manifests
- **Real-time Updates**: Live app store updates
- **Edge Functions**: Custom backend logic

## Setup

### 1. Create Supabase Project

1. Visit [supabase.com](https://supabase.com) and create an account
2. Create a new project
3. Note your project URL and API keys

### 2. Environment Variables

Set these environment variables:

```bash
# Required
export SUPABASE_URL="https://your-project.supabase.co"
export SUPABASE_ANON_KEY="your-anon-key"

# Optional (for admin operations)
export SUPABASE_SERVICE_KEY="your-service-key"
```

### 3. Database Setup

Run the migration files to set up the database schema:

```bash
# Install Supabase CLI
npm install -g @supabase/cli

# Initialize Supabase in your project
supabase init

# Link to your project
supabase link --project-ref your-project-ref

# Run migrations
supabase db push
```

## Database Schema

### Core Tables

#### `profiles`
Extends Supabase auth.users with additional profile information:

```sql
- id: UUID (references auth.users)
- email: TEXT
- username: TEXT
- full_name: TEXT
- avatar_url: TEXT
- role: TEXT (user, moderator, admin)
```

#### `apps`
Main application metadata:

```sql
- id: UUID
- name: TEXT
- slug: TEXT (unique)
- description: TEXT
- version: TEXT
- author_id: UUID
- category: TEXT
- tags: TEXT[]
- manifest_url: TEXT
- wasm_url: TEXT
- size_bytes: BIGINT
- download_count: INTEGER
- is_published: BOOLEAN
```

#### `app_versions`
Version history for applications:

```sql
- id: UUID
- app_id: UUID
- version: TEXT
- changelog: TEXT
- manifest_url: TEXT
- wasm_url: TEXT
- is_latest: BOOLEAN
```

### Security

Row Level Security (RLS) is enabled on all tables with policies that ensure:
- Users can only modify their own data
- Published apps are publicly readable
- Private apps are only visible to authors
- Download analytics respect privacy

## CLI Integration

### Authentication Commands

```bash
# Register new account
wasmrunner register --email user@example.com --username myuser

# Login
wasmrunner login --email user@example.com

# Check login status
wasmrunner whoami
```

### App Store Commands

```bash
# Search apps
wasmrunner search "web server"
wasmrunner search --category web --verified

# Install apps
wasmrunner install nginx-wasm
wasmrunner install http-server:2.1.0

# Publish apps
wasmrunner publish my-app:1.0.0 \
  --description "My awesome WASM app" \
  --category utility \
  --tag cli --tag rust

# Manage favorites
wasmrunner favorite nginx-wasm
wasmrunner favorite nginx-wasm --remove
```

## API Endpoints

The WasmRunner CLI interacts with these Supabase REST API endpoints:

### Authentication
- `POST /auth/v1/signup` - Register new user
- `POST /auth/v1/token` - Login user
- `POST /auth/v1/logout` - Logout user

### Apps
- `GET /rest/v1/apps` - List/search apps
- `POST /rest/v1/apps` - Create new app
- `PATCH /rest/v1/apps?id=eq.{id}` - Update app
- `DELETE /rest/v1/apps?id=eq.{id}` - Delete app

### Favorites
- `GET /rest/v1/user_favorites` - Get user favorites
- `POST /rest/v1/user_favorites` - Add favorite
- `DELETE /rest/v1/user_favorites` - Remove favorite

## Storage Configuration

### Buckets

Create these storage buckets in Supabase:

1. **wasm-files**: Store WASM binaries
   - Public: No
   - File size limit: 50MB
   - Allowed MIME types: `application/wasm`

2. **manifests**: Store app manifests
   - Public: Yes
   - File size limit: 1MB
   - Allowed MIME types: `application/json`

3. **avatars**: User profile pictures
   - Public: Yes
   - File size limit: 2MB
   - Allowed MIME types: `image/*`

### Storage Policies

```sql
-- WASM files readable by authenticated users
CREATE POLICY "WASM files accessible to authenticated users"
  ON storage.objects FOR SELECT
  USING (bucket_id = 'wasm-files' AND auth.role() = 'authenticated');

-- Users can upload to their own folder
CREATE POLICY "Users can upload own WASM files"
  ON storage.objects FOR INSERT
  WITH CHECK (
    bucket_id = 'wasm-files' 
    AND auth.uid()::text = (storage.foldername(name))[1]
  );
```

## Edge Functions

Optional edge functions for advanced functionality:

### `download-app`
Handles app downloads with analytics:

```typescript
import { serve } from 'https://deno.land/std@0.168.0/http/server.ts'
import { createClient } from 'https://esm.sh/@supabase/supabase-js@2'

serve(async (req) => {
  const { appId, version } = await req.json()
  
  // Log download
  await supabase.from('downloads').insert({
    app_id: appId,
    version: version,
    ip_address: req.headers.get('x-forwarded-for'),
    user_agent: req.headers.get('user-agent')
  })
  
  // Return download URL
  const { data } = await supabase.storage
    .from('wasm-files')
    .createSignedUrl(`${appId}/${version}.wasm`, 3600)
    
  return new Response(JSON.stringify({ url: data.signedUrl }))
})
```

### `validate-manifest`
Validates app manifests before publishing:

```typescript
import { serve } from 'https://deno.land/std@0.168.0/http/server.ts'

serve(async (req) => {
  const manifest = await req.json()
  
  // Validate required fields
  const required = ['name', 'version', 'description', 'runtime']
  const missing = required.filter(field => !manifest[field])
  
  if (missing.length > 0) {
    return new Response(
      JSON.stringify({ error: `Missing fields: ${missing.join(', ')}` }),
      { status: 400 }
    )
  }
  
  return new Response(JSON.stringify({ valid: true }))
})
```

## Local Development

For local development, you can run Supabase locally:

```bash
# Start local Supabase
supabase start

# Apply migrations
supabase db reset

# View local dashboard
open http://localhost:54323
```

Set environment variables for local development:

```bash
export SUPABASE_URL="http://localhost:54321"
export SUPABASE_ANON_KEY="your-local-anon-key"
```

## Production Deployment

### Database Optimization

1. **Indexes**: The migration files include optimized indexes for common queries
2. **Connection Pooling**: Use Supabase's built-in connection pooling
3. **Read Replicas**: Enable read replicas for better performance

### Security Best Practices

1. **API Keys**: Never expose service keys in client code
2. **RLS Policies**: Regularly audit Row Level Security policies
3. **Rate Limiting**: Enable rate limiting on authentication endpoints
4. **Monitoring**: Set up alerts for failed authentication attempts

### Backup and Recovery

1. **Daily Backups**: Enable automatic daily backups
2. **Point-in-Time Recovery**: Configure PITR for critical data
3. **Disaster Recovery**: Document recovery procedures

## Troubleshooting

### Common Issues

1. **Authentication Errors**
   - Check SUPABASE_URL and SUPABASE_ANON_KEY
   - Verify user is confirmed (check email)
   - Check RLS policies

2. **Upload Failures**
   - Verify storage bucket exists
   - Check file size limits
   - Verify storage policies

3. **Search Not Working**
   - Check full-text search indexes
   - Verify app is published
   - Check search query format

### Debug Mode

Enable debug logging:

```bash
wasmrunner --verbose search "my app"
```

### Logs and Monitoring

Monitor your Supabase project:
- Database performance
- API usage
- Storage usage
- Authentication metrics
