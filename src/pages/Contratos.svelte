<script lang="ts">
    import { currentUser } from '../stores';
    // El usuario está disponible desde el store
    const user = $currentUser;
</script>

<div class="welcome-container">
    <h1>Panel de Control Principal</h1>
    <p>Bienvenido, {user?.name}. El motor Tauri/Rust está activo.</p>
    <p>Utiliza el menú lateral para navegar a las secciones de **Contratos**, **Salidas** y **Recursos Financieros**.</p>
    <p>Esta página es un placeholder para el dashboard de KPIs de contratos, según el Blueprint.</p>
</div>