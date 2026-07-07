export const PAPER_CART_STORAGE_KEY = 'coscool_paper_cart_v1'

export interface PaperCartItem {
  questionId: number
  addedAt: string
}

interface PaperCartStore {
  items: PaperCartItem[]
}

function normalizeItems(items: PaperCartItem[]) {
  const idSet = new Set<number>()
  const result: PaperCartItem[] = []
  items.forEach((item) => {
    const questionId = Number(item.questionId)
    if (!Number.isFinite(questionId) || questionId <= 0 || idSet.has(questionId)) return
    idSet.add(questionId)
    result.push({
      questionId,
      addedAt: item.addedAt || new Date().toISOString()
    })
  })
  return result
}

export function getPaperCartItems() {
  const raw = localStorage.getItem(PAPER_CART_STORAGE_KEY)
  if (!raw) return []
  try {
    const store = JSON.parse(raw) as PaperCartStore
    return normalizeItems(Array.isArray(store.items) ? store.items : [])
  } catch {
    return []
  }
}

export function getPaperCartIds() {
  return getPaperCartItems().map((item) => item.questionId)
}

export function savePaperCartItems(items: PaperCartItem[]) {
  const store: PaperCartStore = { items: normalizeItems(items) }
  localStorage.setItem(PAPER_CART_STORAGE_KEY, JSON.stringify(store))
  notifyPaperCartChange()
  return store.items
}

export function setPaperCartIds(ids: number[]) {
  const now = new Date().toISOString()
  return savePaperCartItems(ids.map((questionId) => ({ questionId, addedAt: now })))
}

export function addQuestionToPaperCart(questionId: number) {
  const items = getPaperCartItems()
  if (items.some((item) => item.questionId === questionId)) return items
  return savePaperCartItems([...items, { questionId, addedAt: new Date().toISOString() }])
}

export function removeQuestionFromPaperCart(questionId: number) {
  return savePaperCartItems(getPaperCartItems().filter((item) => item.questionId !== questionId))
}

export function clearPaperCart() {
  localStorage.removeItem(PAPER_CART_STORAGE_KEY)
  notifyPaperCartChange()
}

function notifyPaperCartChange() {
  window.dispatchEvent(new CustomEvent('coscool-paper-cart-change'))
}
