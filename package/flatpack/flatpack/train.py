import os
import time
import torch


def train(user_train_function, save_dir, model_type='rnn', framework='pytorch', *args, **kwargs):
    """
    This function is intended to be part of the flatpack package.
    It uses the os and time modules, so these should be imported along with this function.
    """
    os.makedirs(save_dir, exist_ok=True)

    epochs = kwargs.get('epochs', 10)
    batch_size = kwargs.get('batch_size', 32)

    print(f"🚀 Training {model_type} model with epochs: {epochs} and batch size: {batch_size}")

    start_time = time.time()

    result = user_train_function(*args, **kwargs)
    model = result.get('model')

    elapsed_time = time.time() - start_time
    print(f"✅ Training completed in {elapsed_time:.2f} seconds")

    # The torch module is needed for saving the model state
    if framework == 'pytorch' and model is not None:
        torch.save(model.state_dict(), os.path.join(save_dir, f'{model_type}_model.pth'))
