```Tsx
import React from 'react';

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & { variant?: 'primary' | 'secondary' | 'ghost' };

export default function Button({ variant = 'primary', className = '', style, children, ...props }: ButtonProps) {
  const base = 'rounded-md px-4 py-2 font-semibold';
  const variantStyle: React.CSSProperties =
    variant === 'primary'
      ? { background: 'var(--color-primary)', color: 'var(--color-bg)' }
      : variant === 'secondary'
      ? { border: '1px solid var(--color-muted)', color: 'var(--color-text)', background: 'transparent' }
      : { background: 'transparent', color: 'var(--color-primary)' };

  return (
    <button className={`${base} ${className}`} style={{ ...variantStyle, ...style }} {...props}>
      {children}
    </button>
  );
}
```