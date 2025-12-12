import * as React from "react"
import { ChevronDown } from "lucide-react"

export interface SelectProps {
  value?: string
  onValueChange?: (value: string) => void
  placeholder?: string
  children: React.ReactNode
  className?: string
}

export interface SelectItemProps {
  value: string
  children: React.ReactNode
  className?: string
}

const Select = React.forwardRef<HTMLDivElement, SelectProps>(
  ({ value, onValueChange, placeholder, children, className }, ref) => {
    const [isOpen, setIsOpen] = React.useState(false)
    const [selectedValue, setSelectedValue] = React.useState(value || '')
    const selectRef = React.useRef<HTMLDivElement>(null)

    React.useEffect(() => {
      setSelectedValue(value || '')
    }, [value])

    React.useEffect(() => {
      const handleClickOutside = (event: MouseEvent) => {
        if (selectRef.current && !selectRef.current.contains(event.target as Node)) {
          setIsOpen(false)
        }
      }

      document.addEventListener('mousedown', handleClickOutside)
      return () => {
        document.removeEventListener('mousedown', handleClickOutside)
      }
    }, [])

    const handleSelect = (itemValue: string) => {
      setSelectedValue(itemValue)
      setIsOpen(false)
      onValueChange?.(itemValue)
    }

    const getDisplayValue = () => {
      if (!selectedValue) return placeholder || 'Select...'
      
      // 从children中找到对应的SelectItem来获取显示文本
      const findItemText = (children: React.ReactNode): string => {
        let result = selectedValue
        React.Children.forEach(children, (child) => {
          if (React.isValidElement(child) && child.props.value === selectedValue) {
            result = typeof child.props.children === 'string' ? child.props.children : selectedValue
          }
        })
        return result
      }
      
      return findItemText(children)
    }

    return (
      <div ref={selectRef} className={`relative ${className || ''}`}>
        <div
          ref={ref}
          className="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer"
          onClick={() => setIsOpen(!isOpen)}
        >
          <span className={selectedValue ? '' : 'text-muted-foreground'}>
            {getDisplayValue()}
          </span>
          <ChevronDown className={`h-4 w-4 transition-transform ${isOpen ? 'rotate-180' : ''}`} />
        </div>
        
        {isOpen && (
          <div className="absolute top-full left-0 right-0 z-50 mt-1 max-h-60 overflow-auto rounded-md border bg-popover shadow-md">
            <div className="p-1">
              {React.Children.map(children, (child) => {
                if (React.isValidElement(child)) {
                  return React.cloneElement(child as React.ReactElement<SelectItemProps>, {
                    onClick: () => handleSelect(child.props.value),
                    selected: child.props.value === selectedValue
                  })
                }
                return child
              })}
            </div>
          </div>
        )}
      </div>
    )
  }
)
Select.displayName = "Select"

const SelectItem = React.forwardRef<HTMLDivElement, SelectItemProps & { 
  onClick?: () => void
  selected?: boolean 
}>(
  ({ value, children, className, onClick, selected }, ref) => {
    return (
      <div
        ref={ref}
        className={`relative flex w-full cursor-pointer select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none hover:bg-accent hover:text-accent-foreground ${
          selected ? 'bg-accent text-accent-foreground' : ''
        } ${className || ''}`}
        onClick={onClick}
      >
        {children}
      </div>
    )
  }
)
SelectItem.displayName = "SelectItem"

const SelectContent = ({ children }: { children: React.ReactNode }) => {
  return <>{children}</>
}
SelectContent.displayName = "SelectContent"

const SelectTrigger = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => (
    <div
      ref={ref}
      className={`flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${className || ''}`}
      {...props}
    >
      {children}
      <ChevronDown className="h-4 w-4 opacity-50" />
    </div>
  )
)
SelectTrigger.displayName = "SelectTrigger"

const SelectValue = React.forwardRef<HTMLSpanElement, React.HTMLAttributes<HTMLSpanElement> & {
  placeholder?: string
}>(
  ({ className, placeholder, ...props }, ref) => (
    <span
      ref={ref}
      className={`${className || ''}`}
      {...props}
    >
      {placeholder}
    </span>
  )
)
SelectValue.displayName = "SelectValue"

export { Select, SelectItem, SelectContent, SelectTrigger, SelectValue }
