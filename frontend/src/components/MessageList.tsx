import { createEffect, createResource, createSignal, For, Show } from 'solid-js'
import { createVirtualizer } from '@tanstack/solid-virtual'
import { fetchMessages, type Message } from '../api/client'
import { selectedProject, selectedSession, searchQuery, fromTs, toTs } from '../stores/filters'
import { MessageItem } from './MessageItem'

const PAGE_SIZE = 100

export function MessageList() {
  let containerRef: HTMLDivElement | undefined

  const [offset, setOffset] = createSignal(0)
  // Track the offset that is currently loading to prevent duplicate fetches
  const [loadingOffset, setLoadingOffset] = createSignal(-1)

  const params = () => ({
    project: selectedProject(),
    session: selectedSession(),
    q: searchQuery() || undefined,
    from: fromTs(),
    to: toTs(),
    offset: offset(),
    limit: PAGE_SIZE,
  })

  const [data] = createResource(params, fetchMessages)

  const [allMessages, setAllMessages] = createSignal<Message[]>([])
  const [total, setTotal] = createSignal(0)

  // Reset when filters change
  createEffect(() => {
    void [selectedProject(), selectedSession(), searchQuery(), fromTs(), toTs()]
    setOffset(0)
    setLoadingOffset(-1)
    setAllMessages([])
    setTotal(0)
  })

  // Accumulate pages; use the snapshot of offset at response time to decide append vs replace
  createEffect(() => {
    const d = data()
    if (!d) return
    const resolvedOffset = offset()
    if (resolvedOffset === 0) {
      setAllMessages(d.messages)
    } else {
      setAllMessages((prev: Message[]) => [...prev, ...d.messages])
    }
    setTotal(d.total)
    setLoadingOffset(-1)
  })

  const rowVirtualizer = createVirtualizer({
    get count() {
      return allMessages().length
    },
    getScrollElement: () => containerRef ?? null,
    estimateSize: () => 80,
    overscan: 10,
  })

  function onScroll(e: Event) {
    const el = e.target as HTMLDivElement
    const nearBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 300
    const nextOffset = allMessages().length
    if (nearBottom && nextOffset < total() && !data.loading && loadingOffset() !== nextOffset) {
      setLoadingOffset(nextOffset)
      setOffset(nextOffset)
    }
  }

  return (
    <div style={{ flex: 1, display: 'flex', 'flex-direction': 'column', overflow: 'hidden' }}>
      <div style={{ padding: '0.4rem 1rem', background: '#1e1e2e', 'border-bottom': '1px solid #313244', 'font-size': '0.78rem', color: '#6c7086' }}>
        <Show when={!data.loading} fallback="Loading…">
          {total().toLocaleString()} messages
          <Show when={allMessages().length < total() && allMessages().length > 0}>
            {' '}(showing {allMessages().length.toLocaleString()})
          </Show>
        </Show>
      </div>

      <div
        ref={(el) => (containerRef = el)}
        onScroll={onScroll}
        style={{ flex: 1, overflow: 'auto' }}
      >
        <div style={{ height: `${rowVirtualizer.getTotalSize()}px`, position: 'relative' }}>
          <For each={rowVirtualizer.getVirtualItems()}>
            {(vItem) => {
              const msg = () => allMessages()[vItem.index]
              return (
                <Show when={msg()}>
                  <div
                    style={{
                      position: 'absolute',
                      top: 0,
                      left: 0,
                      width: '100%',
                      transform: `translateY(${vItem.start}px)`,
                    }}
                    data-index={vItem.index}
                    ref={(el) => rowVirtualizer.measureElement(el)}
                  >
                    <MessageItem message={msg()} highlight={searchQuery() || undefined} />
                  </div>
                </Show>
              )
            }}
          </For>
        </div>
      </div>

      <Show when={data.loading && allMessages().length > 0}>
        <div style={{ padding: '0.5rem', 'text-align': 'center', color: '#6c7086', 'font-size': '0.8rem' }}>Loading more…</div>
      </Show>
    </div>
  )
}
