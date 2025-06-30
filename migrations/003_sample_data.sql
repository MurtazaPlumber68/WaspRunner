
-- Sample data for WasmRunner App Store
-- Run after the main schema migrations

-- Insert sample categories and apps
DO $$
DECLARE
    sample_user_id UUID;
BEGIN
    -- Create a sample user (you'll need to register this user through Supabase Auth)
    -- This is just for demo purposes
    
    -- Insert sample apps
    INSERT INTO public.apps (
        name, slug, description, long_description, version, 
        author_id, category, tags, manifest_url, wasm_url,
        size_bytes, is_published, is_verified
    ) VALUES 
    (
        'Hello World',
        'hello-world',
        'A simple Hello World WASM application',
        'A basic WebAssembly application that demonstrates the WasmRunner platform. Perfect for testing and learning.',
        '1.0.0',
        (SELECT id FROM auth.users LIMIT 1), -- Use first available user
        'demo',
        ARRAY['demo', 'tutorial', 'beginner'],
        'https://registry.wasmrunner.dev/hello-world/manifest.json',
        'https://registry.wasmrunner.dev/hello-world/hello-world.wasm',
        2048,
        true,
        true
    ),
    (
        'HTTP Server',
        'http-server',
        'Lightweight HTTP server in WASM',
        'A high-performance HTTP server built with Rust and compiled to WebAssembly. Supports static file serving, routing, and middleware.',
        '2.1.0',
        (SELECT id FROM auth.users LIMIT 1),
        'web',
        ARRAY['http', 'server', 'web', 'rust'],
        'https://registry.wasmrunner.dev/http-server/manifest.json',
        'https://registry.wasmrunner.dev/http-server/http-server.wasm',
        1024000,
        true,
        true
    ),
    (
        'JSON Processor',
        'json-processor',
        'Fast JSON processing utility',
        'A command-line JSON processor that can parse, transform, and validate JSON data with high performance.',
        '1.5.2',
        (SELECT id FROM auth.users LIMIT 1),
        'utility',
        ARRAY['json', 'cli', 'parser', 'utility'],
        'https://registry.wasmrunner.dev/json-processor/manifest.json',
        'https://registry.wasmrunner.dev/json-processor/json-processor.wasm',
        512000,
        true,
        false
    );

    -- Update download counts
    UPDATE public.apps SET download_count = 1500 WHERE slug = 'hello-world';
    UPDATE public.apps SET download_count = 850 WHERE slug = 'http-server';
    UPDATE public.apps SET download_count = 320 WHERE slug = 'json-processor';

END $$;
