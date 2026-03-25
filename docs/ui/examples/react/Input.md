```Tsx
import React from 'react';

export default function Input({ className = '', ...props }: React.InputHTMLAttributes<HTMLInputElement>) {
  return (
    <input
      {...props}
      className={`w-full border border-[var(--color-muted)] rounded-md px-3 py-2 bg-[var(--color-surface)] placeholder:text-[var(--color-muted)] ${className}`}
    />
  );
}
```