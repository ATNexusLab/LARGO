```Tsx
import React from 'react';

type Props = { src?: string; alt?: string; className?: string };

export default function Logo({ src = '/docs/design/assets/logo.png', alt = 'Logo', className = '' }: Props) {
  return <img src={src} alt={alt} className={className} />;
}
```