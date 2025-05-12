import React from 'react';
import Image from 'next/image'; // Using Next.js Image component for optimization
import { Heart, Star } from 'lucide-react'; // Import the Star icon

// Define the props the component will accept
interface ListingCardProps {
  imageSrc: string;
  imageAlt: string;
  location: string;
  details?: string; // Optional details like distance or dates
  rating: number;
  price: number; // Price per night
  className?: string; // Allow adding custom classes to the container
}

const ListingCard: React.FC<ListingCardProps> = ({
  imageSrc,
  imageAlt,
  location,
  details,
  rating,
  price,
  className,
}) => {
  return (
    <div className={`cursor-pointer group w-full ${className}`}>

      <div className="relative w-full aspect-[4/3] rounded-xl overflow-hidden">
        <Image
          src={imageSrc}
          alt={imageAlt}
          fill
          sizes="(max-width: 600px) 100vw, (max-width: 1024px) 50vw, 33vw"
          className="object-cover transition-transform duration-300 group-hover:scale-105"

        />
        <div
          className="w-full h-full group-hover:scale-105 bg-lime-300"
        ></div>
        <div className="absolute top-2 right-2">
          <Heart size={24} className="text-white" fill="transparent" />
        </div>
      </div>

      <div className="prose mt-2 text-md">
        <div className="flex justify-between items-start gap-x-2">
          <div className="font-semibold text-black truncate">
            {location}
          </div>
          <div className="flex items-center gap-x-1">
            <Star size={14} className="text-yellow-500 fill-yellow-500" /> {/* Star icon */}
            <span>{rating}</span>
          </div>
        </div>

        {details && (
          <div className="text-gray-500">
            {details}
          </div>
        )}

        <div className="mt-1">
          <span className="font-semibold text-black underline">${price * 5}</span> for 5 nights
        </div>
      </div>
    </div>
  );
};

export default ListingCard;