# Wiki 渲染器

一个用于渲染 Wiki 单条目 JSON 的页面，支持块编辑器风格的文档结构。

## 当前功能

- 手动选择单个 Wiki JSON 文件
- 渲染头图、封面、简介和章节布局
- 渲染文本、表格、列表、图片、引用、分隔线等块类型
- 渲染文本与条目引用等内联元素

## 不再支持

- 从目录批量选择 Wiki / Catalog 文件
- 依赖 `catalog/full.json` 做目录映射
- 在线抓取单条目数据
- 在渲染页内更新 Wiki 数据
- 通过条目引用跳转到其它词条详情页

## 使用方法

1. 进入 `/wiki/render`
2. 选择本地 Wiki JSON 文件
3. 查看渲染结果

## 数据格式

```json
{
  "code": 0,
  "message": "OK",
  "timestamp": "1770138265",
  "data": {
    "item": {
      "itemId": "7",
      "document": {
        "documentMap": {
          "document-id": {
            "id": "document-id",
            "blockIds": ["block1", "block2"],
            "blockMap": {
              "block1": {
                "id": "block1",
                "parentId": "document-id",
                "kind": "text"
              }
            }
          }
        }
      }
    }
  }
}
```

## 组件结构

```text
WikiRendererPage.vue
├── WikiDocument.vue
    └── WikiBlock.vue
        ├── TextBlock.vue
        ├── TableBlock.vue
        ├── ListBlock.vue
        ├── ImageBlock.vue
        ├── QuoteBlock.vue
        ├── HorizontalLineBlock.vue
        └── UnknownBlock.vue
```
