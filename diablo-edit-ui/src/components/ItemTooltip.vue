<script setup lang="ts">
import { computed } from "vue";
import type { D2Item } from "../bindings/D2Item";

const props = defineProps<{
  item: D2Item;
  visible: boolean;
  x: number;
  y: number;
}>();

const itemName = computed(() => {
  if (props.item.data.name) {
    return props.item.data.name;
  }
  const typeId = props.item.data.type_id;
  return String.fromCharCode(...typeId.filter(c => c !== 0));
});

const qualityColor = computed(() => {
  if (!props.item.data.identified) {
    return "#888";
  }
  if (props.item.data.runeword) {
    return "#ffd700";
  }
  if (props.item.data.ethereal) {
    return "#87ceeb";
  }
  return "#fff";
});

const itemProperties = computed(() => {
  const result: string[] = [];
  const data = props.item.data;

  if (!data.identified) {
    result.push("Unidentified");
  }
  if (data.ethereal) {
    result.push("Ethereal");
  }
  if (data.socketed && props.item.socketed_items.length > 0) {
    result.push(`Socketed (${props.item.socketed_items.length})`);
  }
  if (data.personalized) {
    result.push("Personalized");
  }
  if (data.runeword) {
    result.push("Runeword");
  }

  return result;
});

const sockets = computed(() => {
  return props.item.socketed_items.map(gem => {
    const name = gem.data.name || String.fromCharCode(...gem.data.type_id.filter(c => c !== 0));
    return {
      name,
      ethereal: gem.data.ethereal
    };
  });
});

const tooltipStyle = computed(() => ({
  left: `${props.x + 15}px`,
  top: `${props.y + 15}px`,
  display: props.visible ? "block" : "none"
}));
</script>

<template>
  <div class="item-tooltip" :style="tooltipStyle">
    <div class="tooltip-header">
      <span class="item-name" :style="{ color: qualityColor }">{{ itemName }}</span>
    </div>

    <div v-if="itemProperties.length" class="tooltip-props">
      <span
        v-for="(prop, idx) in itemProperties"
        :key="idx"
        class="prop-tag"
      >
        {{ prop }}
      </span>
    </div>

    <div v-if="sockets.length" class="tooltip-sockets">
      <div class="sockets-label">Sockets:</div>
      <div
        v-for="(socket, idx) in sockets"
        :key="idx"
        class="socket-item"
      >
        {{ socket.name }}
        <span v-if="socket.ethereal" class="ethereal-tag">(E)</span>
      </div>
    </div>

    <div class="tooltip-size">
      Size: {{ item.data.width }}x{{ item.data.height }}
    </div>
  </div>
</template>

<style scoped>
.item-tooltip {
  position: fixed;
  z-index: 1000;
  background: #1a1a1a;
  border: 1px solid #444;
  border-radius: 6px;
  padding: 10px 12px;
  min-width: 150px;
  max-width: 280px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  pointer-events: none;
  font-size: 13px;
}

.tooltip-header {
  margin-bottom: 8px;
  padding-bottom: 6px;
  border-bottom: 1px solid #333;
}

.item-name {
  font-weight: bold;
  font-size: 14px;
}

.tooltip-props {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 8px;
}

.prop-tag {
  font-size: 11px;
  padding: 2px 6px;
  background: #333;
  border-radius: 3px;
  color: #aaa;
}

.ethereal-tag {
  color: #87ceeb;
  font-size: 10px;
}

.tooltip-sockets {
  margin-bottom: 8px;
  padding-top: 6px;
  border-top: 1px solid #333;
}

.sockets-label {
  font-size: 11px;
  color: #888;
  margin-bottom: 4px;
}

.socket-item {
  font-size: 12px;
  color: #ccc;
  padding-left: 8px;
}

.tooltip-size {
  font-size: 11px;
  color: #666;
  margin-top: 6px;
  padding-top: 6px;
  border-top: 1px solid #333;
}
</style>
