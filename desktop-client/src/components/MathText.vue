<template>
  <div class="math-text">
    <template v-for="(token, index) in tokens" :key="index">
      <span v-if="token.type === 'text'" class="math-text-content">{{ token.value }}</span>
      <span v-else-if="token.error" class="math-source">{{ token.raw }}</span>
      <span
        v-else-if="token.display"
        class="math-block"
        v-html="token.html"
      />
      <span
        v-else
        class="math-inline"
        v-html="token.html"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import katex from 'katex'

interface TextToken {
  type: 'text'
  value: string
}

interface MathToken {
  type: 'math'
  value: string
  raw: string
  display: boolean
  html: string
  error: boolean
}

type Token = TextToken | MathToken
type Delimiter = '$' | '$$'

const props = defineProps<{
  content?: string
}>()

const tokens = computed(() => parseMathText(props.content || ''))

function parseMathText(content: string) {
  const result: Token[] = []
  let currentIndex = 0

  while (currentIndex < content.length) {
    const nextDelimiter = findNextDelimiter(content, currentIndex)
    if (!nextDelimiter) {
      appendText(result, content.slice(currentIndex))
      break
    }

    if (nextDelimiter.index > currentIndex) {
      appendText(result, content.slice(currentIndex, nextDelimiter.index))
    }

    const closeIndex = findDelimiter(content, nextDelimiter.delimiter, nextDelimiter.index + nextDelimiter.delimiter.length)
    if (closeIndex === -1) {
      appendText(result, content.slice(nextDelimiter.index))
      break
    }

    const formula = content.slice(nextDelimiter.index + nextDelimiter.delimiter.length, closeIndex)
    const raw = content.slice(nextDelimiter.index, closeIndex + nextDelimiter.delimiter.length)
    result.push(renderFormula(formula, raw, nextDelimiter.delimiter === '$$'))
    currentIndex = closeIndex + nextDelimiter.delimiter.length
  }

  return result
}

function appendText(tokens: Token[], value: string) {
  if (!value) return
  const lastToken = tokens[tokens.length - 1]
  if (lastToken?.type === 'text') {
    lastToken.value += value
    return
  }
  tokens.push({ type: 'text', value })
}

function findNextDelimiter(content: string, startIndex: number): { delimiter: Delimiter; index: number } | null {
  const blockIndex = findDelimiter(content, '$$', startIndex)
  const inlineIndex = findDelimiter(content, '$', startIndex)

  if (blockIndex === -1 && inlineIndex === -1) return null
  if (blockIndex !== -1 && (inlineIndex === -1 || blockIndex <= inlineIndex)) {
    return { delimiter: '$$', index: blockIndex }
  }
  return { delimiter: '$', index: inlineIndex }
}

function findDelimiter(content: string, delimiter: Delimiter, startIndex: number) {
  let index = content.indexOf(delimiter, startIndex)
  while (index !== -1) {
    if (!isEscaped(content, index) && (delimiter === '$$' || content[index + 1] !== '$')) {
      return index
    }
    index = content.indexOf(delimiter, index + delimiter.length)
  }
  return -1
}

function isEscaped(content: string, index: number) {
  let slashCount = 0
  for (let cursor = index - 1; cursor >= 0 && content[cursor] === '\\'; cursor -= 1) {
    slashCount += 1
  }
  return slashCount % 2 === 1
}

function renderFormula(value: string, raw: string, display: boolean): MathToken {
  try {
    return {
      type: 'math',
      value,
      raw,
      display,
      html: katex.renderToString(value, {
        displayMode: display,
        output: 'html',
        strict: 'ignore',
        throwOnError: true,
        trust: false
      }),
      error: false
    }
  } catch {
    return {
      type: 'math',
      value,
      raw,
      display,
      html: '',
      error: true
    }
  }
}
</script>

<style scoped>
.math-text {
  white-space: pre-wrap;
  word-break: break-word;
}

.math-text-content {
  white-space: pre-wrap;
}

.math-inline {
  display: inline;
}

.math-block {
  display: block;
  max-width: 100%;
  margin: 8px 0;
  overflow-x: auto;
  overflow-y: hidden;
}

.math-source {
  color: #d4380d;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
  white-space: pre-wrap;
}
</style>
