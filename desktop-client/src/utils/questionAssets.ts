import type { AssetDataUrl, Question } from '../api/native'

type ReadAsset = (relativePath: string) => Promise<AssetDataUrl>

export function getOptionImageKey(questionId: number, optionKey: string) {
  return `${questionId}:${optionKey}`
}

export async function buildQuestionImageMap(questions: Question[], readAssetDataUrl: ReadAsset) {
  const imageMap: Record<number, string> = {}
  await Promise.all(
    questions.map(async (question) => {
      if (!question.imageText || !question.imageText.startsWith('assets/')) return
      try {
        const asset = await readAssetDataUrl(question.imageText)
        imageMap[question.id] = asset.dataUrl
      } catch {
        imageMap[question.id] = ''
      }
    })
  )
  return imageMap
}

export async function buildOptionImageMap(questions: Question[], readAssetDataUrl: ReadAsset) {
  const imageMap: Record<string, string> = {}
  await Promise.all(
    questions.flatMap((question) =>
      question.options.map(async (option) => {
        if (!option.imageText || !option.imageText.startsWith('assets/')) return
        try {
          const asset = await readAssetDataUrl(option.imageText)
          imageMap[getOptionImageKey(question.id, option.optionKey)] = asset.dataUrl
        } catch {
          imageMap[getOptionImageKey(question.id, option.optionKey)] = ''
        }
      })
    )
  )
  return imageMap
}
