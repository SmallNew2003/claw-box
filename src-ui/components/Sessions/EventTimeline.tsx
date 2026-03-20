import type { RuntimeEvent } from '../../../src/shared/session-runtime';

interface EventTimelineProps {
  events: RuntimeEvent[];
}

export function EventTimeline({ events }: EventTimelineProps) {
  return (
    <section className="panel timeline-panel">
      <h3>运行日志</h3>
      {events.length === 0 ? (
        <div className="empty">暂无事件</div>
      ) : (
        <ul className="timeline-list">
          {events.map((event) => (
            <li key={event.event_id} className="timeline-item">
              <div className="timeline-head">
                <span className={`event-tag ${event.type}`}>{event.type}</span>
                <time>{event.created_at}</time>
              </div>
              <code>{JSON.stringify(event.payload)}</code>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
