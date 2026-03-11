<template>
  <div class="wiki-table-wrapper">
    <table class="wiki-table">
      <tbody>
        <tr v-for="rowId in block.table.rowIds" :key="rowId">
          <template v-for="colId in block.table.columnIds" :key="`${rowId}_${colId}`">
            <td
              v-if="getCell(rowId, colId)"
              v-bind="getCellProps(rowId, colId)"
              :style="getCellStyle(rowId, colId)"
            >
              <template v-for="entry in getCellBlockEntries(rowId, colId)" :key="entry.id">
                <WikiBlock :block="entry.block" :block-map="blockMap" />
              </template>
            </td>
          </template>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar';
import WikiBlock from '../WikiBlock.vue';
import type { TableBlock, Block } from '../../../types/wiki';
import { COLOR_MAP, DARK_COLOR_MAP } from '../../../types/wiki';

const props = defineProps<{
  block: TableBlock;
  blockMap: Record<string, Block>;
}>();

const $q = useQuasar();

function resolveRawColor(color: string, isDark: boolean): string {
  if (isDark && DARK_COLOR_MAP[color]) {
    return DARK_COLOR_MAP[color];
  }
  return COLOR_MAP[color] || color;
}

function getCell(rowId: string, colId: string) {
  return props.block.table.cellMap[`${rowId}_${colId}`];
}

function getCellProps(rowId: string, colId: string) {
  const cell = getCell(rowId, colId);
  if (!cell) return {};

  const cellProps: Record<string, number> = {};

  if (cell.rowSpan && cell.rowSpan !== '1') {
    cellProps.rowspan = parseInt(cell.rowSpan, 10);
  }

  if (cell.colSpan && cell.colSpan !== '1') {
    cellProps.colspan = parseInt(cell.colSpan, 10);
  }

  return cellProps;
}

function getCellStyle(rowId: string, colId: string) {
  const cell = getCell(rowId, colId);
  const column = props.block.table.columnMap[colId];

  if (!cell) return {};

  const style: Record<string, string> = {};

  if (column?.width) {
    style.width = `${column.width}px`;
  }

  if (cell.backgroundColor) {
    style.backgroundColor = resolveRawColor(cell.backgroundColor, $q.dark.isActive);
  }

  if (cell.borderColor) {
    style.borderColor = resolveRawColor(cell.borderColor, $q.dark.isActive);
  }

  if (cell.verticalAlign && cell.verticalAlign !== 'unknown') {
    style.verticalAlign = cell.verticalAlign;
  }

  return style;
}

function getCellChildren(rowId: string, colId: string): string[] {
  const cell = getCell(rowId, colId);
  return cell?.childIds || [];
}

function getCellBlockEntries(rowId: string, colId: string): Array<{ id: string; block: Block }> {
  return getCellChildren(rowId, colId)
    .map((id) => ({ id, block: props.blockMap[id] }))
    .filter((entry): entry is { id: string; block: Block } => Boolean(entry.block));
}
</script>

<style scoped lang="scss">
.wiki-table-wrapper {
  margin: 1em 0;
  overflow-x: auto;
}

.wiki-table {
  width: 100%;
  border-collapse: collapse;
  border-spacing: 0;

  td {
    border: 1px solid var(--table-border-color);
    padding: 8px;
    vertical-align: top;
  }
}
</style>
