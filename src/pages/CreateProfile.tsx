import ConfirmationDialog from '@/components/ConfirmationDialog';
import Header from '@/components/Header';
import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { amount } from '@/lib/formTypes';
import { zodResolver } from '@hookform/resolvers/zod';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { useNavigate } from 'react-router-dom';
import * as z from 'zod';
import { commands, Error, TransactionResponse } from '../bindings';
import Container from '../components/Container';
import ErrorDialog from '../components/ErrorDialog';
import { useWalletState } from '../state';
import { TokenAmountInput } from '@/components/ui/masked-input';

export default function CreateProfile() {
  const navigate = useNavigate();
  const walletState = useWalletState();

  const [error, setError] = useState<Error | null>(null);
  const [response, setResponse] = useState<TransactionResponse | null>(null);

  const formSchema = z.object({
    name: z.string().min(1, 'Name is required'),
    fee: amount(walletState.sync.unit.decimals).optional(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  const onSubmit = (values: z.infer<typeof formSchema>) => {
    commands
      .createDid({ name: values.name, fee: values.fee?.toString() || '0' })
      .then((result) => {
        if (result.status === 'error') {
          console.error(result.error);
          setError(result.error);
          return;
        } else {
          setResponse(result.data);
        }
      });
  };

  return (
    <>
      <Header title='Create Profile' />

      <Container className='max-w-xl'>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-4'>
            <FormField
              control={form.control}
              name='name'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input placeholder='Display name' {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className='grid sm:grid-cols-2 gap-4'>
              <FormField
                control={form.control}
                name='fee'
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Network Fee</FormLabel>
                    <FormControl>
                      <div className='relative'>
                        <TokenAmountInput {...field} className='pr-12' />
                        <div className='pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3'>
                          <span className='text-gray-500 text-sm'>
                            {walletState.sync.unit.ticker}
                          </span>
                        </div>
                      </div>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <Button type='submit'>Create Profile</Button>
          </form>
        </Form>
      </Container>

      <ErrorDialog error={error} setError={setError} />
      <ConfirmationDialog
        response={response}
        close={() => setResponse(null)}
        onConfirm={() => navigate('/dids')}
      />
    </>
  );
}
