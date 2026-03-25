```Tsx
import React from 'react';

type Props = { value?: number; className?: string };

export default function ProgressElectric({ value = 0, className = '' }: Props) {
  const v = Math.max(0, Math.min(100, value));
  return (
    <div className={`progress progress-electric ${className}`} role="progressbar" aria-valuenow={v} aria-valuemin={0} aria-valuemax={100}>
      <div className="progress-value" style={{ width: `${v}%` }} />
    </div>
  );
}
```