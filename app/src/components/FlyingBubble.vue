<script setup lang="ts">
const props = defineProps<{
  message: string;
  offsetX?: number;
}>();
</script>

<template>
  <div class="bubble" :style="{ '--x': ((props.offsetX ?? 0) + 'px') }">
    {{ props.message }}
  </div>
</template>

<style scoped>
.bubble {
  --x: 0px;
  position: absolute;
  bottom: 0;
  left: 50%;
  padding: 6px 10px;
  border-radius: 26px;
  background: #842528;
  color: #eee;
  font-size: 0.9rem;
  white-space: nowrap;
  pointer-events: none;
  transform: translate(calc(-50% + var(--x, 0px)), 0);
  animation: fly-up 1s ease-in-out forwards;
}

@keyframes fly-up {
  0%   { transform: translate(calc(-50% + var(--x, 0px)), 0); opacity: 0; }
  65%  { opacity: 1; }
  100% { transform: translate(calc(-50% + var(--x, 0px)), -50px); opacity: 0; }
}
</style>
