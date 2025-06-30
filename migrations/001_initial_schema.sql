
-- WasmRunner App Store Database Schema
-- Run with: supabase db push

-- Enable Row Level Security
ALTER DATABASE postgres SET "app.jwt_secret" TO 'your-jwt-secret-here';

-- Users table (extends Supabase auth.users)
CREATE TABLE public.profiles (
    id UUID REFERENCES auth.users(id) PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE,
    full_name TEXT,
    avatar_url TEXT,
    role TEXT DEFAULT 'user' CHECK (role IN ('user', 'moderator', 'admin')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Applications table
CREATE TABLE public.apps (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT,
    long_description TEXT,
    version TEXT NOT NULL DEFAULT '0.1.0',
    author_id UUID REFERENCES public.profiles(id) NOT NULL,
    category TEXT DEFAULT 'utility',
    tags TEXT[] DEFAULT '{}',
    
    -- WASM-specific fields
    manifest_url TEXT NOT NULL,
    wasm_url TEXT NOT NULL,
    size_bytes BIGINT DEFAULT 0,
    runtime_requirements JSONB DEFAULT '{}',
    
    -- Metadata
    homepage_url TEXT,
    repository_url TEXT,
    license TEXT DEFAULT 'MIT',
    
    -- Stats
    download_count INTEGER DEFAULT 0,
    star_count INTEGER DEFAULT 0,
    
    -- Status
    is_published BOOLEAN DEFAULT false,
    is_verified BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- App versions table for version history
CREATE TABLE public.app_versions (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    app_id UUID REFERENCES public.apps(id) ON DELETE CASCADE,
    version TEXT NOT NULL,
    changelog TEXT,
    manifest_url TEXT NOT NULL,
    wasm_url TEXT NOT NULL,
    size_bytes BIGINT DEFAULT 0,
    
    -- Release info
    is_prerelease BOOLEAN DEFAULT false,
    is_latest BOOLEAN DEFAULT false,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(app_id, version)
);

-- User favorites
CREATE TABLE public.user_favorites (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE,
    app_id UUID REFERENCES public.apps(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(user_id, app_id)
);

-- Download analytics
CREATE TABLE public.downloads (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    app_id UUID REFERENCES public.apps(id) ON DELETE CASCADE,
    version TEXT NOT NULL,
    user_id UUID REFERENCES public.profiles(id),
    ip_address INET,
    user_agent TEXT,
    platform TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- App reviews and ratings
CREATE TABLE public.reviews (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    app_id UUID REFERENCES public.apps(id) ON DELETE CASCADE,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    title TEXT,
    content TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(app_id, user_id)
);

-- Create indexes for performance
CREATE INDEX idx_apps_author ON public.apps(author_id);
CREATE INDEX idx_apps_category ON public.apps(category);
CREATE INDEX idx_apps_published ON public.apps(is_published);
CREATE INDEX idx_apps_featured ON public.apps(is_featured);
CREATE INDEX idx_apps_created ON public.apps(created_at);
CREATE INDEX idx_app_versions_app ON public.app_versions(app_id);
CREATE INDEX idx_downloads_app ON public.downloads(app_id);
CREATE INDEX idx_downloads_created ON public.downloads(created_at);
CREATE INDEX idx_reviews_app ON public.reviews(app_id);

-- Full-text search index for apps
CREATE INDEX idx_apps_search ON public.apps USING gin(
    to_tsvector('english', name || ' ' || COALESCE(description, ''))
);
