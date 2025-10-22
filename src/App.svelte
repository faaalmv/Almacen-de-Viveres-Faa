<script lang="ts">
  // Recibe un componente y sus propiedades para renderizarlos dinámicamente.
  export let component: any;
  export let props: Record<string, any> = {};
</script>

<svelte:component this={component} {...props} />