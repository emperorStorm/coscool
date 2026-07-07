<template>
  <section class="paper-preview">
    <div class="paper-head">
      <h1>{{ title || '未命名试卷' }}</h1>
      <div class="paper-info">
        <span>姓名：________________</span>
        <span>班级：________________</span>
        <span>日期：________________</span>
      </div>
    </div>

    <div v-if="questions.length" class="question-list">
      <article v-for="(question, index) in questions" :key="question.id" class="paper-question">
        <div class="question-title">
          <strong>{{ index + 1 }}.</strong>
          <MathText :content="question.stem || question.title || '暂无题干'" />
        </div>
        <img v-if="imageMap[question.id]" class="question-image" :src="imageMap[question.id]" alt="题目配图" />
        <p v-else-if="question.imageText" class="image-placeholder">{{ question.imageText }}</p>
        <div v-if="question.options.length && isChoiceQuestion(question.questionType)" class="option-grid">
          <div v-for="option in question.options" :key="option.optionKey" class="option-item">
            <strong>{{ option.optionKey }}.</strong>
            <MathText :content="option.content || '-'" />
          </div>
        </div>
        <div v-if="showAnswer || showAnalysis" class="answer-block">
          <div v-if="showAnswer">
            <strong>答案：</strong>
            <MathText :content="question.answer || '暂无答案'" />
          </div>
          <div v-if="showAnalysis">
            <strong>解析：</strong>
            <MathText :content="question.analysis || '暂无解析'" />
          </div>
        </div>
        <div v-if="showKnowledgePoints || showTags" class="paper-meta-block">
          <div v-if="showKnowledgePoints" class="meta-line">
            <strong>知识点</strong>
            <span v-for="item in question.knowledgePoints" :key="item">{{ item }}</span>
            <em v-if="!question.knowledgePoints.length">暂无</em>
          </div>
          <div v-if="showTags" class="meta-line">
            <strong>标签</strong>
            <span v-for="item in question.tags" :key="item">{{ item }}</span>
            <em v-if="!question.tags.length">暂无</em>
          </div>
        </div>
      </article>
    </div>

    <a-empty v-else description="暂无题目" />
  </section>
</template>

<script setup lang="ts">
import MathText from './MathText.vue'
import type { Question } from '../api/native'

defineProps<{
  title: string
  questions: Question[]
  imageMap: Record<number, string>
  showAnswer?: boolean
  showAnalysis?: boolean
  showKnowledgePoints?: boolean
  showTags?: boolean
}>()

function isChoiceQuestion(questionType: string) {
  return questionType === '单选题' || questionType === '多选题'
}
</script>

<style scoped>
.paper-preview {
  width: min(800px, 100%);
  min-height: 1120px;
  margin: 0 auto;
  padding: 42px 48px;
  color: #1f2937;
  background: #ffffff;
  border: 1px solid #dfe8f4;
  border-radius: 8px;
  box-shadow: 0 18px 42px rgba(42, 60, 84, 0.10);
}

.paper-head {
  padding-bottom: 18px;
  border-bottom: 2px solid #1f2937;
}

.paper-head h1 {
  margin: 0;
  color: #111827;
  font-size: 24px;
  font-weight: 900;
  line-height: 1.4;
  text-align: center;
}

.paper-info {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 12px;
  margin-top: 18px;
  color: #374151;
  font-size: 14px;
  font-weight: 700;
}

.question-list {
  display: grid;
  gap: 24px;
  margin-top: 28px;
}

.paper-question {
  page-break-inside: avoid;
}

.question-title {
  display: flex;
  gap: 8px;
  align-items: flex-start;
  color: #1f2937;
  font-size: 16px;
  font-weight: 800;
  line-height: 1.8;
}

.question-title > strong {
  flex: 0 0 auto;
  min-width: 22px;
}

.question-title :deep(.math-text) {
  min-width: 0;
}

.question-image {
  display: block;
  max-width: 100%;
  max-height: 280px;
  margin: 14px 0 0 30px;
  object-fit: contain;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
}

.image-placeholder {
  margin: 12px 0 0 30px;
  color: #8792a2;
  font-size: 13px;
}

.option-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px 22px;
  margin: 16px 0 0 30px;
}

.option-item {
  display: flex;
  gap: 7px;
  min-width: 0;
  color: #273244;
  font-size: 15px;
  line-height: 1.7;
}

.option-item strong {
  flex: 0 0 auto;
}

.option-item :deep(.math-text) {
  min-width: 0;
}

.answer-block {
  display: grid;
  gap: 8px;
  margin: 18px 0 0 30px;
  padding: 12px 14px;
  background: #f6faf9;
  border: 1px solid #dcebe8;
  border-radius: 8px;
}

.answer-block > div {
  display: flex;
  gap: 8px;
  align-items: flex-start;
  color: #243144;
  font-size: 14px;
  line-height: 1.7;
}

.answer-block strong {
  flex: 0 0 auto;
  color: #0f9187;
}

.paper-meta-block {
  display: grid;
  gap: 8px;
  margin: 14px 0 0 30px;
}

.meta-line {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  color: #526174;
  font-size: 13px;
}

.meta-line strong {
  color: #2388e8;
}

.meta-line span {
  display: inline-flex;
  align-items: center;
  min-height: 24px;
  padding: 0 9px;
  color: #385dd6;
  font-weight: 700;
  background: #eef1ff;
  border: 1px solid #cbd6ff;
  border-radius: 999px;
}

.meta-line em {
  color: #8792a2;
  font-style: normal;
}

@media (max-width: 760px) {
  .paper-preview {
    min-height: auto;
    padding: 28px 22px;
  }

  .paper-info,
  .option-grid {
    grid-template-columns: 1fr;
  }
}
</style>
