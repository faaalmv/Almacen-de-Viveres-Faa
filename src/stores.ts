// src/stores.ts (CONTENIDO AUTO-CONTENIDO)
import { writable } from 'svelte/store';

// Define el tipo de datos del usuario, siguiendo el modelo de la sección 2.2 y 2.4.
// Usamos string para el rol por ahora, tal como se menciona en la sección 3.3.
export interface User {
    id: string;
    name: string;
    role: 'Validador' | 'Proveedor' | 'Administrador' | 'Usuario';
}

// El store del usuario actual. Inicialmente nulo (no autenticado).
export const currentUser = writable<User | null>(null);