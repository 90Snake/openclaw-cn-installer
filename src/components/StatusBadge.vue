<script setup lang="ts">
defineProps<{
  tone: "ok" | "warn" | "danger" | "neutral";
  text: string;
}>();
</script>

<template>
  <span class="badge" :data-tone="tone">{{ text }}</span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 999px;
  padding: 0.35rem 0.8rem;
  font-size: 0.85rem;
  font-weight: 700;
  letter-spacing: 0.02em;
}

.badge[data-tone="ok"] {
  background: rgba(41, 153, 78, 0.14);
  color: #1c6b34;
}

.badge[data-tone="warn"] {
  background: rgba(200, 120, 14, 0.14);
  color: #8b4c00;
}

.badge[data-tone="danger"] {
  background: rgba(190, 52, 52, 0.14);
  color: #941d1d;
}

.badge[data-tone="neutral"] {
  background: rgba(18, 56, 102, 0.1);
  color: #183e67;
}
</style>
