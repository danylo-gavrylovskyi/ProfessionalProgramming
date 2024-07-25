use object_pool::mega_data::MegaData;
use object_pool::mega_data_pool::MegaDataPool;

#[test]
fn test_create_mega_data_with_default_values() {
    let data = MegaData::new();

    for &val in data.small_array.iter() {
        assert_eq!(val, 42.0);
    }

    for &val in data.big_array.iter() {
        assert_eq!(val, 42.0);
    }
}

#[test]
fn test_reset_mega_data_to_default_values() {
    let mut data = MegaData::new();
    data.small_array[0] = 0.0;
    data.big_array[0] = 0.0;
    
    data.reset();

    assert_eq!(data.small_array[0], 42.0);
    assert_eq!(data.big_array[0], 42.0);
}

#[test]
fn test_initialize_mega_data_pool_with_correct_size() {
    let pool = MegaDataPool::new(10);
    assert_eq!(pool.size(), 10);
    assert_eq!(pool.used_size(), 0);
    assert_eq!(pool.available_indices.len(), 10);
}

#[test]
fn test_acquire_and_release_mega_data_objects() {
    let mut pool = MegaDataPool::new(10);
    
    {
        let data = pool.acquire().unwrap();
        assert_eq!(pool.used_size(), 1);
        assert_eq!(pool.available_indices.len(), 9);

        pool.release(data);
    }

    assert_eq!(pool.used_size(), 0);
    assert_eq!(pool.available_indices.len(), 10);
}

#[test]
fn test_return_none_when_pool_exhausted() {
    let mut pool = MegaDataPool::new(1);
    let _data = pool.acquire().unwrap();
    let result = pool.acquire();
    assert!(result.is_none());
}
