<script lang="ts">
    import "$lib/app.css";

    import * as Sidebar from "$lib/components/ui/sidebar";

    import AppSidebar from "$lib/components/app-sidebar";

    let { children } = $props();
</script>

<Sidebar.Provider>
    <AppSidebar />
    <main class="container mx-auto px-2">
        <Sidebar.Trigger />

        {@render children?.()}
    </main>
</Sidebar.Provider>
