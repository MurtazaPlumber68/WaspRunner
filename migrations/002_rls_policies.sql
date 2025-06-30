
-- Row Level Security Policies for WasmRunner

-- Enable RLS on all tables
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.apps ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.app_versions ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.user_favorites ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.downloads ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.reviews ENABLE ROW LEVEL SECURITY;

-- Profiles policies
CREATE POLICY "Public profiles are viewable by everyone" 
    ON public.profiles FOR SELECT 
    USING (true);

CREATE POLICY "Users can update own profile" 
    ON public.profiles FOR UPDATE 
    USING (auth.uid() = id);

-- Apps policies
CREATE POLICY "Published apps are viewable by everyone" 
    ON public.apps FOR SELECT 
    USING (is_published = true);

CREATE POLICY "Authors can view their own apps" 
    ON public.apps FOR SELECT 
    USING (auth.uid() = author_id);

CREATE POLICY "Authors can update their own apps" 
    ON public.apps FOR UPDATE 
    USING (auth.uid() = author_id);

CREATE POLICY "Authenticated users can create apps" 
    ON public.apps FOR INSERT 
    WITH CHECK (auth.uid() = author_id);

-- App versions policies
CREATE POLICY "App versions viewable if app is viewable" 
    ON public.app_versions FOR SELECT 
    USING (
        EXISTS (
            SELECT 1 FROM public.apps 
            WHERE apps.id = app_versions.app_id 
            AND (apps.is_published = true OR apps.author_id = auth.uid())
        )
    );

CREATE POLICY "Authors can manage their app versions" 
    ON public.app_versions FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM public.apps 
            WHERE apps.id = app_versions.app_id 
            AND apps.author_id = auth.uid()
        )
    );

-- User favorites policies
CREATE POLICY "Users can view their own favorites" 
    ON public.user_favorites FOR SELECT 
    USING (auth.uid() = user_id);

CREATE POLICY "Users can manage their own favorites" 
    ON public.user_favorites FOR ALL
    USING (auth.uid() = user_id);

-- Downloads policies (logged anonymously)
CREATE POLICY "Downloads are insertable by everyone" 
    ON public.downloads FOR INSERT 
    WITH CHECK (true);

CREATE POLICY "Download stats viewable by app authors" 
    ON public.downloads FOR SELECT 
    USING (
        EXISTS (
            SELECT 1 FROM public.apps 
            WHERE apps.id = downloads.app_id 
            AND apps.author_id = auth.uid()
        )
    );

-- Reviews policies
CREATE POLICY "Reviews are viewable by everyone for published apps" 
    ON public.reviews FOR SELECT 
    USING (
        EXISTS (
            SELECT 1 FROM public.apps 
            WHERE apps.id = reviews.app_id 
            AND apps.is_published = true
        )
    );

CREATE POLICY "Users can manage their own reviews" 
    ON public.reviews FOR ALL
    USING (auth.uid() = user_id);

-- Create functions for automatic profile creation
CREATE OR REPLACE FUNCTION public.handle_new_user() 
RETURNS trigger AS $$
BEGIN
    INSERT INTO public.profiles (id, email, username)
    VALUES (new.id, new.email, split_part(new.email, '@', 1));
    RETURN new;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Trigger for automatic profile creation
CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE PROCEDURE public.handle_new_user();

-- Function to update app download count
CREATE OR REPLACE FUNCTION update_download_count()
RETURNS trigger AS $$
BEGIN
    UPDATE public.apps 
    SET download_count = download_count + 1
    WHERE id = NEW.app_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER on_download_created
    AFTER INSERT ON public.downloads
    FOR EACH ROW EXECUTE PROCEDURE update_download_count();
