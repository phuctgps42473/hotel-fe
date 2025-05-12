interface Category {
  value: string;
  label: string;
}

interface CategoryFilterBarProps {
  categories: Category[];
  onSelectCategory: (categoryValue: string | null) => void;
  onOpenFilters?: () => void;
  initialCategory?: string | null;
}

const CategoryFilterBar: React.FC<CategoryFilterBarProps> = ({
  categories,
  onSelectCategory,
  onOpenFilters,
  initialCategory = null,
}) => {
  const [activeCategory, setActiveCategory] = useState<string | null>(initialCategory);

  const handleCategoryClick = (categoryValue: string) => {
    setActiveCategory(categoryValue);
    onSelectCategory(categoryValue);
  };

  return (
    <div className="w-full px-10 flex shadow-sm py-4 overflow-hidden">
      <div className="no-scrollbar flex items-center overflow-x-auto whitespace-nowrap px-4 md:px-6 scrollbar-hide">
        {categories.map((category) => {
          const isActive = activeCategory === category.value;
          return (
            <button
              key={category.value}
              className={`
                flex-shrink-0
                flex items-center
                px-4 py-2
                mx-1
                rounded-full
                text-sm
                border
                transition-colors
                cursor-pointer
                ${isActive
                  ? 'border-yellow-300 font-semibold bg-primary text-primary-content'
                  : 'border-gray-300 hover:border-gray-400'
                }
              `}
              onClick={() => handleCategoryClick(category.value)}
            >
              <span>{category.label}</span>
            </button>
          );
        })}

      </div>
      {onOpenFilters && (
        <button
          className="flex-shrink-0 flex items-center px-4 py-2 ml-4 rounded-full text-sm border border-gray-300 hover:border-gray-400 transition-colors"
          onClick={onOpenFilters}
        >
          <SlidersHorizontal size={16} className="mr-2" />
          Filters
        </button>
      )}
    </div>
  );
};

export default CategoryFilterBar;