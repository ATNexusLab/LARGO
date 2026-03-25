```Tsx
import React from 'react';

export default function Card({ className = '', children, ...props }: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={`bg-[var(--color-surface)] rounded-md shadow-md p-4 ${className}`} {...props}>
      {children}
    </div>
  );
}
```