# Solana Task Manager

Proyecto backend desarrollado en Rust para Solana.

## Descripción

Este programa implementa un CRUD de tareas en la blockchain de Solana utilizando Program Derived Addresses (PDA).

Cada tarea pertenece a un usuario y se almacena en una cuenta derivada de su public key.

## Funcionalidades

- Crear tarea
- Actualizar tarea
- Completar tarea
- Eliminar tarea

## Tecnologías

- Solana
- Rust
- Anchor

## PDA

Las cuentas utilizan el seed:

task + user_public_key
