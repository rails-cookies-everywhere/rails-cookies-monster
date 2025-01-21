
class MonstersController < ActionController::Base
  def cookies_monster
    cookies[:regular] = ENV['CANARY_VALUE']
    cookies.signed[:signed] = ENV['CANARY_VALUE']
    cookies.encrypted[:encrypted] = ENV['CANARY_VALUE']
    session[:session] = ENV['CANARY_VALUE']
    head :ok
  end
end
